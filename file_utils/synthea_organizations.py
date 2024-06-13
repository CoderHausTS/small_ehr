import pandas as pd
import numpy as np

organizations_csv = "csv/organizations.csv"

df = pd.read_csv(organizations_csv)

# Just put a blank where there's no value. This will import as such, but we can be OK for testing.
df.fillna('', inplace=True)

# Get the old columns out of here
df.drop(columns = ['Id'], inplace=True)
df.rename(columns = str.lower, inplace=True)
df.to_csv("csv/organiations_cleaned.csv", index=False)
