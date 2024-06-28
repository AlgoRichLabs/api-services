from enum import Enum


class SIDE(Enum):
    LONG = "long"
    SHORT = "short"
    BUY = "buy"
    SELL = "sell"


class EXCHANGE(Enum):
    OKX = "okx"
    BINANCE = "binance"
