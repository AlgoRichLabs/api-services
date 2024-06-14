import requests
import os
import pandas as pd

# Example
# TODO: add a script argument.
symbol = "MOAT"
url = f'https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={symbol}&outputsize=full&apikey=67TE0X14Y8PJ4X3V'
r = requests.get(url)
data = r.json()

# Extract the time series data
time_series = data.get('Time Series (Daily)', {})

# Convert the time series data into a pandas DataFrame
df = pd.DataFrame.from_dict(time_series, orient='index')

# Rename the columns to make them more readable
df.columns = ['Open', 'High', 'Low', 'Close', 'Volume']

# Add a date column
df.index.name = 'Date'

output_dir = f"data/{symbol}"
os.makedirs(output_dir, exist_ok=True)
output_file_path = os.path.join(output_dir, f'{symbol}.csv')
df.to_csv(output_file_path)