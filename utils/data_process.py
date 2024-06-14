import pandas as pd

def read_symbol(symbol):
    df = pd.read_csv(f"data/{symbol}/{symbol}.csv")
    df['Date'] = pd.to_datetime(df['Date'])
    return df.sort_values(by="Date")