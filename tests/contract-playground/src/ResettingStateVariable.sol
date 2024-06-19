//SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

contract ResettingStateVariable {
    uint256 public myNumber1;
    uint256 public myNumber2;

    // function setNumberWithoutChek(uint256 _num) public {
    //     myNumber1 = _num;
    // }

    function setNumberWithChek1(uint256 _num) public {
        if (myNumber1 != _num) {
            myNumber1 = _num;
        }
    }

    function anotherWithouthChek(uint256 _num) public {
        myNumber2 = _num;
    }

    function setNumberWithChek2(uint256 _num) public {
        if (myNumber1 != _num) {
            myNumber1 = _num;
        }
    }
}
