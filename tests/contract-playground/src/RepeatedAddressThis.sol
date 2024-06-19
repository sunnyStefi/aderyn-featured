// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract AddressThisExample {
    function perform() external view returns (address) {
        address test1 = address(this);
    }

    function performTooMuch() external view returns (address) {
        address test = address(this);
        return test;
    }
}
