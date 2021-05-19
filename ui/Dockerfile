FROM node:lts-alpine

COPY ["package.json", "/opt/ui/"]
# COPY ["package-lock.json", "/opt/ui/"]

WORKDIR /opt/ui

RUN npm install

ENV PATH /opt/ui/node_modules/.bin:$PATH

COPY ["src/", "/opt/ui/src/"]
COPY ["public/", "/opt/ui/public/"]

EXPOSE 3000

CMD ["npm", "start"]
