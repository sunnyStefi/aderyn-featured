// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract AnotherOne {
    function callMe() external returns (address) {}
}

contract AddressThisExample {
    AnotherOne anotherOne;

    function perform() external returns (address) {
        address test1 = address(this);
        address test2 = anotherOne.callMe();
    }

    function performTooMuch() external view returns (address) {
        address test = address(this);
        return test;
    }
}
