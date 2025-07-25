# Dioxus AI Test Project

This is a test project for trying out dioxus and using AI tools.
AI generated commits are marked in the commit message.

The project is deployed to GitHub Pages automatically: https://tomok.github.io/dioxus-ai-test/

The following parts of this file are from the README generated by dx.

## Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # The entrypoint for the app.
│  ├─ components/
│  │  ├─ mod.rs # Defines the components module
│  │  ├─ hero.rs # The Hero component for use in the home page
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Tailwind
1. Enter the Nix development environment: `nix develop`
2. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```
**WARNING**: Depending on the browser library on the desktop, the resulting application might not render svgs and hence not show any graph.

## Git Hooks

This project includes Git hooks to enforce code quality standards. See the [hooks directory](./hooks/README.md) for setup instructions and available hooks.

## Deployment

This project is automatically deployed to GitHub Pages using GitHub Actions. Every push to the main branch triggers a build and deployment process:

1. The GitHub Actions workflow builds the Dioxus application
2. The built application is deployed to GitHub Pages
3. The deployed application can be accessed at: https://tomok.github.io/dioxus-ai-test/

To manually deploy, you can trigger the workflow from the Actions tab in the GitHub repository.

