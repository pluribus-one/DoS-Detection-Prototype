import pandas as pd
import csv 
log_file_path = 'archive/access.log'
output_file = "features.csv"
column_names = ["ip","timestamp"]
rows = []
# Open the file and read it line by line
count = 0
with open(log_file_path, 'r') as file:
    
    for line in file:
        
        # Process each line as needed
        # For example, split the line into fields using spaces
        try:
            fields = line.split("\"")[0].split("-")
        except:
            print(f"line n {count} -- malformated : {fields}")
        # Perform operations on the fields
        
        if len(fields) >= 3:
            rows.append([fields[0],fields[2]])
        else:
            print(f"line n {count} -- malformated : {fields}")
            
        count += 1
        
 
# Open the CSV file in write mode
with open(output_file, 'w', newline='') as file:
    # Create a CSV writer object
    csv_writer = csv.writer(file)
    
    # Write column names to the CSV file
    csv_writer.writerow(column_names)
    
    # Write rows to the CSV file
    csv_writer.writerows(rows)
