//SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

contract ResettingStateVariable {
    function stackVar1IsUsedOnce(uint256 _num) public {
        address stackVar1 = msg.sender;
        address stackVar2 = stackVar1;
    }

    function stackVar1IsUsedMoreTimes(uint256 _num) public {
        address stackVar1 = msg.sender;
        address stackVar2 = stackVar1;
        address stackVar3 = stackVar1;
        address stackVar4 = stackVar1;
    }
}
