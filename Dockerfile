ARG NODE_VERSION
FROM node:$NODE_VERSION

ARG APP_DIR

RUN mkdir -p $APP_DIR

WORKDIR $APP_DIR

COPY package.json .
RUN npm install --silent
RUN npm install --global mocha
RUN npm install request
COPY . .
EXPOSE 3000

ENV PORT 3000

CMD [ "npm", "start" ]