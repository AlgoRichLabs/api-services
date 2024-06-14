from utils.order import Order
class Position:
    def __init__(self, symbol) -> None:
        self.symbol = symbol
        self.amount = 0
        self.position_value = 0
        self.unrealized_pnl = 0
        self.average_entry_price = None
    

    def order_filled(self, order: Order):
        if order.side == "sell":
            self.amount -= order.quantity
            if self.amount < 0:
                raise Exception("Does not support shorting a position.")
        elif order.side == "buy":
            # Does not consider shorting a position now
            if not self.average_entry_price:
                self.average_entry_price = order.filled_price
            else:
                self.average_entry_price = (self.amount * self.average_entry_price + order.quantity * order.filled_price) \
                / (self.amount + order.quantity)
            self.amount += order.quantity
        else:
            raise Exception("Invalid order side.")
        self.update_position(order.filled_price)
    
    def update_position(self, price: float):
        self.unrealized_pnl = (self.average_entry_price - price) * self.amount
        self.position_value = self.amount * price



        