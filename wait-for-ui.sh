#!/bin/bash
# wait-for-ui.sh

# wait for a response from the ui
echo -n "Waiting for the ui to finish initializing..."
while [ "$(curl -s -o /dev/null -w "%{http_code}" -m 1 localhost:3000)" != "200" ]
do
  sleep 1
  echo -n "."
done

# print message
cat <<EOF

The client is ready!
Open localhost:3000 in your browser to view the ui.
Run 'make logs' to view the service logs.
Run 'make stop' to stop the docker containers.
Run 'make help' to view other available commands.
EOF
