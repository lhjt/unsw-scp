# Install dependencies only when needed
FROM node:18.2.0-alpine AS deps
RUN npm i -g pnpm
# Check https://github.com/nodejs/docker-node/tree/b4117f9333da4138b03a546ec926ef50a31506c3#nodealpine to understand why libc6-compat might be needed.
RUN apk add --no-cache libc6-compat
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN pnpm i --frozen-lockfile

# If using npm with a `package-lock.json` comment out above and use below instead
# COPY package.json package-lock.json ./ 
# RUN npm ci

# Rebuild the source code only when needed
FROM node:18.2.0-alpine AS builder
RUN npm i -g pnpm
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# Next.js collects completely anonymous telemetry data about general usage.
# Learn more here: https://nextjs.org/telemetry
# Uncomment the following line in case you want to disable telemetry during the build.
# ENV NEXT_TELEMETRY_DISABLED 1

RUN pnpm build && pnpm export

# If using npm comment out above and use below instead
# RUN npm run build

# Production image, copy all the files and run next
FROM nginx:alpine
# Copy the local nginx configuration folder
COPY nginx /etc/nginx
# Set the working directory to the default nginx html directory
WORKDIR /usr/share/nginx/html
# Remove the existing web files
RUN rm -rf ./*
# Copy the files from the static next export
COPY --from=builder /app/out /usr/share/nginx/html