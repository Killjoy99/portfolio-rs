# My Rust Portfolio app

## Development
```bash
# Run locally
cargo run

# Build release
cargo build --release
```

## Deployment

### Prerequisites
- A Linux server with nginx installed
- Domain `portfolio.entweni.com` pointing to your server

### Manual Deployment

1. **Setup server** (run on server):
```bash
chmod +x scripts/server-setup.sh
./scripts/server-setup.sh
```

2. **Transfer files**:
```bash
scp -r release/philani-portfolio templates/ static/ .env* portfolio.db \
  user@portfolio.entweni.com:/var/www/portfolio/
```

3. **Start app**:
```bash
ssh user@portfolio.entweni.com
cd /var/www/portfolio
chmod +x philani-portfolio
sudo systemctl start portfolio
sudo systemctl status portfolio
```

4. **Get SSL certificate**:
```bash
sudo certbot --nginx -d portfolio.entweni.com
```

### GitHub Actions Deployment (Recommended)

Push a git tag to trigger automatic deployment:

```bash
# Create and push a tag
git tag v1.0.0
git push origin v1.0.0
```

**Required GitHub Secrets:**
- `SERVER_HOST` - Server IP or domain
- `SERVER_USER` - SSH username
- `SERVER_SSH_KEY` - Private SSH key

### Quick Commands
```bash
# Check app status
sudo systemctl status portfolio

# View logs
sudo journalctl -u portfolio -f

# Restart app
sudo systemctl restart portfolio
```
