// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Counter.sol";

contract CounterTest is Test {
    Counter public counter;

    function setUp() public {
        counter = new Counter();
    }

    function test_storesOwner() public {
        assertEq(counter.getOwner(), address(this));
    }

    function test_cannnotBeAbove10() public {
        for (int i = 0; i <= 10; i++) {
            counter.increment();
        }

        assertEq(counter.getCount(), 0);
    }

    function test_cannotBeBelow0() public {
        counter.decrement();

        assertEq(counter.getCount(), 10);
    }

    function test_resetByOwner() public {
        counter.increment();
        counter.increment();
        counter.reset();
        assertEq(counter.getCount(), 0);
    }

    function testFail_resetByNonOwner() public {
        counter.increment();
        counter.increment();
        vm.prank(address(0));
        counter.reset();
    }
}
