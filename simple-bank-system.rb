# https://leetcode.com/problems/simple-bank-system

class Bank

=begin
    :type balance: Integer[]
=end
    def initialize(balance)
        @balance = balance
        @balance.unshift(nil)
    end


=begin
    :type account1: Integer
    :type account2: Integer
    :type money: Integer
    :rtype: Boolean
=end
    def transfer(account1, account2, money)
        return false unless @balance[account1]
        return false unless @balance[account2]
        return false if @balance[account1] < money

        @balance[account1] -= money
        @balance[account2] += money

        true
    end


=begin
    :type account: Integer
    :type money: Integer
    :rtype: Boolean
=end
    def deposit(account, money)
        return false unless @balance[account]

        @balance[account] += money
        true
    end


=begin
    :type account: Integer
    :type money: Integer
    :rtype: Boolean
=end
    def withdraw(account, money)
        return false unless @balance[account]
        return false if @balance[account] < money

        @balance[account] -= money
        true
    end


end

# Your Bank object will be instantiated and called as such:
# obj = Bank.new(balance)
# param_1 = obj.transfer(account1, account2, money)
# param_2 = obj.deposit(account, money)
# param_3 = obj.withdraw(account, money)