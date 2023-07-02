# Use the official Node.js image as the base image
FROM node:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the source code to the container
COPY . .

# Install dependencies
RUN npm install

# Build the application
RUN npm run build

EXPOSE 3000

# Set the command to run your application
CMD ["npm", "start"]
