// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "openzeppelin/access/Ownable.sol";

contract Counter is Ownable {
    uint8 public value;

    constructor() {
        value = 0;
    }

    /**
     * @dev Resets the counter to 0 and can only be called by the contract
     * owner.
     */
    function reset() public onlyOwner {
        value = 0;
    }

    /**
     * @dev Increment the counter by 1 and reset it to 0 if it exceeds 10.
     */
    function increment() public {
        if (value <= 10) {
            value++;
        } else {
            value = 0;
        }
    }

    /**
     * @dev Decrement the counter by 1 and reset it to 10 if it goes below 0.
     */
    function decrement() public {
        if (value >= 0) {
            value--;
        } else {
            value = 10;
        }
    }

    /**
     * @dev Get the current value of the counter.
     */
    function getCount() public view returns (uint256) {
        return value;
    }

    /**
     * @dev Get the owner of the contract.
     */
    function getOwner() public view returns (address) {
        return owner();
    }
}
