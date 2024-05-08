# budgeting
The tool in server/scripts allows a bank report file (.csv) to be placed in server/scripts/files/report.csv and it will prompt the user to insert categories for the descriptions available in the report. <br>
A template for how the report file should be formatted is shown in server/scripts/files/reportTemplate.csv <br>

Once the user selects a category for a description, it is placed in server/scripts/files/cache.json, which will be used to avoid asking the user for the same descriptions in the future. <br>

## Usage - CLI
Insert the report file (report.csv) in the specified directory (server/scripts/files) and with the format specified in the "reportTemplate.csv" file. <br><br>
To run the CLI application, run the following command from the "server" directory: <br>
```deliverables/budgeting cli```

## In development
Using a Spring-Boot maven application, create a webpage that allows a user to upload the report and receive analytics on it.