#!/bin/bash

# Server Setup Script for Portfolio App
# Run this on your server as root or with sudo

set -e

echo "🔧 Setting up server for portfolio application..."

# Update system
echo "📦 Updating system packages..."
apt update && apt upgrade -y

# Install required packages
echo "📦 Installing nginx, certbot, and dependencies..."
apt install -y nginx certbot python3-certbot-nginx git curl

# Create application directory
echo "📁 Creating application directory..."
mkdir -p /var/www/portfolio-rs
chown -R $USER:$USER /var/www/portfolio-rs

# Create systemd service
echo "⚙️  Creating systemd service..."
cat > /etc/systemd/system/portfolio-rs.service << 'EOF'
[Unit]
Description=Philani Portfolio Rust Application
After=network.target

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=/var/www/portfolio-rs
Environment="DOTENV=/var/www/portfolio-rs/.env"
ExecStart=/var/www/portfolio-rs/philani-portfolio
Restart=always
RestartSec=5
SyslogIdentifier=portfolio-rs

[Install]
WantedBy=multi-user.target
EOF

echo "⚙️  Creating nginx configuration..."
cat > /etc/nginx/sites-available/portfolio-rs << 'EOF'
server {
    listen 80;
    server_name portfolio.entweni.com www.portfolio.entweni.com;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name portfolio.entweni.com www.portfolio.entweni.com;

    ssl_certificate /etc/letsencrypt/live/portfolio.entweni.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/portfolio.entweni.com/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;

    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css text/xml text/javascript application/x-javascript application/xml application/javascript application/json;

    location /static/ {
        alias /var/www/portfolio-rs/static/;
        expires 30d;
        add_header Cache-Control "public, immutable";
    }

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_http_version 1.1;
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }
}
EOF

# Enable nginx site
ln -sf /etc/nginx/sites-available/portfolio-rs /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default
nginx -t

# Configure firewall
echo "🔥 Configuring firewall..."
ufw allow OpenSSH
ufw allow 'Nginx Full'
ufw --force enable

# Reload services
systemctl daemon-reload
systemctl enable nginx

echo ""
echo "✅ Server setup complete!"
echo ""
echo "Next steps:"
echo "1. Transfer your application files to /var/www/portfolio-rs"
echo "2. Create .env file with your configuration"
echo "3. Start the app: sudo systemctl start portfolio"
echo "4. Get SSL: sudo certbot --nginx -d portfolio.entweni.com"
echo "5. Verify: curl https://portfolio.entweni.com"

