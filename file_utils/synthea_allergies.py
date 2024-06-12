import pandas as pd
import numpy as np

allergies_csv = "csv/allergies.csv"

df = pd.read_csv(allergies_csv)

# Just put a blank where there's no value. This will import as such, but we can be OK for testing.
df.fillna('', inplace=True)

# Not ideal, this is brute force. But for our testing, this should be fine
df['SNOMED'] = ("( " + df[['REACTION1', 'DESCRIPTION1', 'SEVERITY1']]
   .astype(str)
   .agg(', '.join, axis=1).astype(str) + "), " +
   "( "  + df[['REACTION2', 'DESCRIPTION2', 'SEVERITY2']]
   .astype(str)
   .agg(', '.join, axis=1).astype(str) + ")" )

# Get the old columns out of here
df.drop(columns = ['REACTION1', 'DESCRIPTION2', 'SEVERITY1', 'REACTION2', 'DESCRIPTION2', 'SEVERITY2'], inplace=True)
df.rename(columns = str.lower, inplace=True)

# get all UUIDs from patient file and randomly plce them in this file


df.to_csv("csv/allergies_cleaned.csv", index=False)
