import pandas as pd
import numpy as np

providers_csv = "csv/providers.csv"

df = pd.read_csv(providers_csv)

# Just put a blank where there's no value. This will import as such, but we can be OK for testing.
df.fillna('', inplace=True)

# Get the old columns out of here
df.drop(columns = ['Id'], inplace=True)
df.rename(columns = str.lower, inplace=True)
df.to_csv("csv/providers_cleaned.csv", index=False)

# There's more work to be done! Providers need to be linked to an organization,
# which doesn't happen here. In data cleaning, we'd do a few things. In our 
# test here we'd do this by hand.
