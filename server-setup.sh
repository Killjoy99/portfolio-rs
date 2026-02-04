#!/bin/bash
set -e

DOMAIN="portfolio.entweni.com"
APP_DIR="/var/www/portfolio-rs"
SERVICE_NAME="portfolio-rs"
BIN_NAME="portfolio-rs"

echo "🔧 Setting up server for Portfolio Rust App..."

# -----------------------------
# 1. Install dependencies
# -----------------------------
echo "📦 Installing system packages..."
apt update
apt install -y nginx certbot python3-certbot-nginx git curl ufw

# -----------------------------
# 2. App directory
# -----------------------------
echo "📁 Creating application directory..."
mkdir -p $APP_DIR

# Copy app files (assumes script is run from project root)
cp -r templates static $APP_DIR
cp release/$BIN_NAME $APP_DIR

touch $APP_DIR/.env
touch $APP_DIR/portfolio.db

chmod +x $APP_DIR/$BIN_NAME
chown -R www-data:www-data $APP_DIR

# -----------------------------
# 3. systemd service
# -----------------------------
echo "⚙️ Creating systemd service..."

tee /etc/systemd/system/$SERVICE_NAME.service > /dev/null << EOF
[Unit]
Description=Philani Portfolio Rust Application
After=network.target

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=$APP_DIR
Environment="DOTENV=$APP_DIR/.env"
ExecStart=$APP_DIR/$BIN_NAME
Restart=always
RestartSec=5
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=full
ProtectHome=true

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable $SERVICE_NAME
systemctl start $SERVICE_NAME

# -----------------------------
# 4. Nginx HTTP config (TEMP)
# -----------------------------
echo "🌐 Creating temporary HTTP nginx config..."

tee /etc/nginx/sites-available/$SERVICE_NAME > /dev/null << EOF
server {
    listen 80;
    server_name $DOMAIN www.$DOMAIN;

    location /static/ {
        alias $APP_DIR/static/;
        expires 30d;
    }

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}
EOF

ln -sf /etc/nginx/sites-available/$SERVICE_NAME /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default

nginx -t
systemctl enable nginx

if systemctl is-active --quiet nginx; then
    systemctl reload nginx
else
    systemctl start nginx
fi

# -----------------------------
# 5. Firewall
# -----------------------------
echo "🔥 Configuring firewall..."
ufw allow OpenSSH
ufw allow 'Nginx Full'
ufw --force enable

# -----------------------------
# 6. SSL with Certbot
# -----------------------------
echo "🔐 Obtaining SSL certificate..."
certbot --nginx -d $DOMAIN -d www.$DOMAIN --non-interactive --agree-tos -m admin@$DOMAIN

# -----------------------------
# 7. Hardened HTTPS config
# -----------------------------
echo "🔒 Applying HTTPS nginx config..."

tee /etc/nginx/sites-available/$SERVICE_NAME > /dev/null << EOF
server {
    listen 80;
    server_name $DOMAIN www.$DOMAIN;
    return 301 https://\$host\$request_uri;
}

server {
    listen 443 ssl http2;
    server_name $DOMAIN www.$DOMAIN;

    ssl_certificate /etc/letsencrypt/live/$DOMAIN/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/$DOMAIN/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;

    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types
        text/plain
        text/css
        application/json
        application/javascript
        application/xml
        application/xml+rss
        text/javascript;

    location /static/ {
        alias $APP_DIR/static/;
        expires 30d;
        add_header Cache-Control "public, immutable";
    }

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_http_version 1.1;
        proxy_connect_timeout 60s;
        proxy_read_timeout 60s;
    }
}
EOF

nginx -t
systemctl reload nginx

# -----------------------------
# Done
# -----------------------------
echo ""
echo "✅ Deployment complete!"
echo ""
echo "🔗 https://$DOMAIN"
echo ""
echo "Useful commands:"
echo "systemctl status $SERVICE_NAME"
echo "journalctl -u $SERVICE_NAME -f"
