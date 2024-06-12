import pandas as pd
import numpy as np

patients_csv = "csv/patients.csv"

df = pd.read_csv(patients_csv)

# Just put a blank where there's no value. This will import as such, but we can be OK for testing.
df.fillna('', inplace=True)

# Get the old columns out of here
df.drop(columns = ['Id'], inplace=True)
df.rename(columns = str.lower, inplace=True)
df.to_csv("csv/patients_cleaned.csv", index=False)
