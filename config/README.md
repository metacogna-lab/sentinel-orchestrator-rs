# Environment Configuration

This directory contains environment configuration templates for Sentinel Orchestrator.

## Setup Instructions

### Development Environment

1. Copy the development template to the project root:
   ```bash
   cp config/.env.development.template .env.development
   ```

2. Edit `.env.development` and fill in your actual values:
   - Set `OPENAI_API_KEY` to your OpenAI API key
   - Update database passwords if needed
   - Adjust other settings as necessary

### Production Environment

1. Copy the production template to the project root:
   ```bash
   cp config/.env.production.template .env.production
   ```

2. **IMPORTANT**: For production, use GitHub Secrets or AWS Secrets Manager instead of committing `.env.production`:
   - Set secrets in GitHub repository settings
   - Or use AWS Secrets Manager for Lightsail deployments
   - Never commit actual secrets to version control

3. Edit `.env.production` with production values:
   - Use strong passwords
   - Set proper CORS origins
   - Configure production domains
   - Set up backup S3 credentials

## Environment File Loading Order

The application loads environment variables in this order (later files override earlier ones):

1. `.env.development` or `.env.production` (based on `ENVIRONMENT` variable)
2. `.env.development.local` or `.env.production.local` (local overrides)
3. `.env.local` (general local overrides)
4. `.env` (fallback for backward compatibility)
5. System environment variables (highest priority)

## Environment Flag

Set the `ENVIRONMENT` variable to control which configuration is loaded:

- `ENVIRONMENT=development` - Loads `.env.development`
- `ENVIRONMENT=production` - Loads `.env.production`
- Default: `development` if not set

## Deployment

### AWS Lightsail (Recommended for Backend)

AWS Lightsail is recommended for the Rust backend because:
- Simple container deployment
- Integrated load balancer
- Easy database setup
- Cost-effective for small to medium deployments

### Cloudflare Pages (Recommended for Frontend)

Cloudflare Pages is recommended for the React frontend because:
- Fast global CDN
- Automatic HTTPS
- Easy environment variable management
- Free tier for static sites

## GitHub Actions

The CI/CD pipelines automatically:
- Use development environment for `develop` branch
- Use production environment for `main` branch
- Load secrets from GitHub Secrets
- Build and deploy to appropriate environments

