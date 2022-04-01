# Rust Blockchain

A simple block chain app that can be access via a CLI.

## Demo

```
Input a miner address: minecrafter
Input difficulty: 2
generating genesis block...
Block hash: 005093ecaf7364926df28aace786e3e4d6b177757516f68d2276a4ded2bd5
Block {
    header: Blockheader {
        timestamp: 1648794819805,
        nonce: 17714,
        pre_hash: "0000000000000000000000000000000000000000000000000000000000000000",
        merkle: "d5da751b1bd5aaf92986ff4b345fe1b3d1ddab1ba7aee2a5075d899dff9c544",
        difficulty: 2,
    },
    count: 1,
    transactions: [
        Transaction {
            sender: "Root",
            receiver: "minecrafter",
            amount: 100.0,
        },
    ],
}
Menu
1) New Transaction
2) Mine block
3) Change Difficulty
4) Change Reward
0) Exit
Enter your choice:
```