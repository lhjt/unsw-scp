# Elastic's Next.js EUI Starter


Jump right in to building prototypes with [EUI](https://github.com/elastic/eui).

_🚩 This starter is not constantly maintained and may get out of sync with the latest EUI release._

## Deployment

### Environment Variables

| Variable | Description | Default |
| -- | -- | -- |
| `GAIA_ADDR` | The address at which gaia is accessible at. | `gaia-backend:8081` |
|`ROUTER_URL` | The address at which the routing service is available at. | `router:8082` |
|`BASE_DOMAIN` | The public base domain on which the proxy is running. | `local.host:8443` |


## 🚀 Quick start

1.  **Install yarn**

    This starter expects to use [yarn](https://yarnpkg.com/) to manage
    dependencies, so go install it.

1.  **Copy the Next.js starter**

    Clone the repository:

    ```sh
    git clone https://github.com/elastic/next-eui-starter.git my-eui-starter
    ```

1.  **Start developing.**

    Navigate into your new site’s directory and start it up.

    ```sh
    cd my-eui-starter/

    # Install depdendencies.
    yarn

    # Optional: start a new git project
    rm -rf .git && git init && git add . && git commit -m "Initial commit"

    # Start the dev server
    yarn dev
    ```

1.  **Open the source code and start editing!**

    Your site is now running at `http://localhost:3000`!

    Open the `my-eui-starter` directory in your code editor of choice and edit `src/pages/index.tsx`. Save your changes and the browser will update in real time!

---

## 🧐 What's inside?

A quick look at the top-level files and directories you'll see in this project.

    .
    ├── .eslintrc.js
    ├── .gitignore
    ├── .next/
    ├── .prettierrc
    ├── LICENSE
    ├── README.md
    ├── next.config.js
    ├── node_modules/
    ├── package.json
    ├── public/
    ├── src/
    ├── tsconfig.json
    └── yarn.lock

1.  **`.eslintrc.js`**: This file configures [ESLint](https://eslint.org/), which will check the code for potential problems and style issues. It also integrates with Prettier for formatting.

2.  **`.gitignore`**: This file tells git which files it should not track / not maintain a version history for.

3.  **`.next`**: The `next` command line tool uses this for various purposes. You should never need to touch it, but you can delete it without causing any problems.

4.  **`.prettierrc`**: This is a configuration file for [Prettier](https://prettier.io/). Prettier is a tool to help keep the formatting of your code consistent.

5.  **`LICENSE`**: Next.js is licensed under the MIT license.

6.  **`README.md`**: A text file containing useful reference information about your project.

7.  **`next.config.js`**: This file customizes the Next.js build process so that it can work with EUI.

8.  **`node_modules/`**: This directory contains all of the modules of code that your project depends on (npm packages) are automatically installed.

9. **`package.json`**: A manifest file for Node.js projects, which includes things like metadata (the project’s name, author, etc). This manifest is how npm knows which packages to install for your project.

10. **`public/`**: Files that will never change can be put here. This starter project automatically puts EUI theme files here during the build

11. **`src/`**: This directory will contain all of the code related to what you will see on the front-end of your site (what you see in the browser) such as your site header or a page template. `src` is a convention for “source code”.

12. **`tsconfig.json`**: This file configures the [TypeScript](https://www.typescriptlang.org/) compiler

13. **`yarn.lock`** (See `package.json` below, first). This is an automatically generated file based on the exact versions of your npm dependencies that were installed for your project. **(You won’t change this file directly, but you need to commit any changes to git).**

## 🎓 Learning Next.js

Looking for more guidance? Full documentation for Next.js lives [on the website](https://nextjs.org/). You probably want to being by following the [Getting Started Guide](https://nextjs.org/learn/basics/getting-started).

## Other features

* Bundle analysis - run `yarn analyze` and two windows will open in your browser, showing how big your server and client bundles are, and where that data is coming from. You can use this information to work out where you're sending too much data to the client, and speed up your pages.
