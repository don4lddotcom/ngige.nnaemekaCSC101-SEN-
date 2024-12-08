
# Importing necessary libraries
import datetime
import random
import string
import sqlite3

# Connecting to the SQLite database
conn = sqlite3.connect('banking_system.db')
cursor = conn.cursor()

# Creating tables in the database
cursor.execute('''
    CREATE TABLE IF NOT EXISTS customers
    (customer_id INTEGER PRIMARY KEY, name TEXT, email TEXT, phone TEXT, address TEXT)
''')

cursor.execute('''
    CREATE TABLE IF NOT EXISTS accounts
    (account_id INTEGER PRIMARY KEY, customer_id INTEGER, account_type TEXT, balance REAL, FOREIGN KEY (customer_id) REFERENCES customers (customer_id))
''')

cursor.execute('''
    CREATE TABLE IF NOT EXISTS transactions
    (transaction_id INTEGER PRIMARY KEY, account_id INTEGER, transaction_type TEXT, amount REAL, timestamp TEXT, FOREIGN KEY (account_id) REFERENCES accounts (account_id))
''')

# Defining a class for customers
class Customer:
    def __init__(self, customer_id, name, email, phone, address):
        self.customer_id = customer_id
        self.name = name
        self.email = email
        self.phone = phone
        self.address = address

    def create_account(self, account_type, initial_balance):
        cursor.execute('INSERT INTO accounts (customer_id, account_type, balance) VALUES (?, ?, ?)', (self.customer_id, account_type, initial_balance))
        conn.commit()
        return cursor.lastrowid

    def get_accounts(self):
        cursor.execute('SELECT * FROM accounts WHERE customer_id = ?', (self.customer_id,))
        return cursor.fetchall()

    def deposit(self, account_id, amount):
        cursor.execute('SELECT balance FROM accounts WHERE account_id = ?', (account_id,))
        balance = cursor.fetchone()[0]
        new_balance = balance + amount
        cursor.execute('UPDATE accounts SET balance = ? WHERE account_id = ?', (new_balance, account_id))
        conn.commit()
        self.log_transaction(account_id, 'deposit', amount)

    def withdraw(self, account_id, amount):
        cursor.execute('SELECT balance FROM accounts WHERE account_id = ?', (account_id,))
        balance = cursor.fetchone()[0]
        if balance >= amount:
            new_balance = balance - amount
            cursor.execute('UPDATE accounts SET balance = ? WHERE account_id = ?', (new_balance, account_id))
            conn.commit()
            self.log_transaction(account_id, 'withdrawal', amount)
        else:
            print('Insufficient funds')

    def log_transaction(self, account_id, transaction_type, amount):
        timestamp = datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')
        cursor.execute('INSERT INTO transactions (account_id, transaction_type, amount, timestamp) VALUES (?, ?, ?, ?)', (account_id, transaction_type, amount, timestamp))
        conn.commit()

# Defining a class for bank employees
class Employee:
    def __init__(self, employee_id, name, email, phone):
        self.employee_id = employee_id
        self.name = name
        self.email = email
        self.phone = phone

    def create_customer(self, name, email, phone, address):
        cursor.execute('INSERT INTO customers (name, email, phone, address) VALUES (?, ?, ?, ?)', (name, email, phone, address))
        conn.commit()
        return cursor.lastrowid

    def get_customers(self):
        cursor.execute('SELECT * FROM customers')
        return cursor.fetchall()

    def get_customer(self, customer_id):
        cursor.execute('SELECT * FROM customers WHERE customer_id = ?', (customer_id,))
        return cursor.fetchone()

# Defining a function to generate a random account number
def generate_account_number():
    return ''.join(random.choices(string.digits, k=16))

# Defining a function to generate a random customer ID
def generate_customer_id():
    return ''.join(random.choices(string.digits, k=10))

# Defining a function to generate a random employee ID
def generate_employee_id():
    return ''.join(random.choices(string.digits, k=8))

# Defining a main function to run the application
def main():
    print('Welcome to the banking system!')

    while True:
        print('1. Create customer')
        print('2. Create account')
        print('3. Deposit')
        print('4. Withdraw')
        print('5. Get customer info')
        print('6. Get account info')
        print('7. Exit')

        choice = input('Enter choice: ')

        if choice == '1':
            name = input('Enter customer name: ')
            email = input('Enter customer email: ')
            phone = input('Enter customer phone: ')
            address = input('Enter customer address: ')
            employee = Employee(generate_employee_id(), 'John Doe', 'john.doe@example.com', '123-456-7890')
            customer_id = employee.create