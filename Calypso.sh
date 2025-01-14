#!/bin/bash
export TRELLO_KEY=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX # Replace XXXXXXX... with your Trello key
export TRELLO_TOKEN=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX # Replace XXXXXXX... with your Trello token
export BOARD_ID=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX # Replace XXXXXXX... with your Trello board ID
export FORM_URL=https://docs.google.com/forms/d/e/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX/formResponse # Replace XXXXXXX... with your google form ID

# Debugging outputs
echo "TRELLO_KEY: $TRELLO_KEY"
echo "TRELLO_TOKEN: $TRELLO_TOKEN"
echo "BOARD_ID: $BOARD_ID"
echo "FORM_URL: $FORM_URL"

# Run the Rust program
/home/PATH/TO/YOUR/PROJECT/FOLDER