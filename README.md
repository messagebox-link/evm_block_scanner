# BlockScanLib

### 说明
该库用于获取以太坊系的区块信息，交易详情，最新区块高度

### 使用指南
- 可配置参数

 1. 设置指定RPC 节点集
 2. 设置链ID 方便使用测试网或者BSC等
 3. 设置重试，默认为`3`

- 功能
  1. 获取最新区块高度 `get_block_latest_number()`
  2. 获取指定区块信息 `get_transactions_for_block(number)`
  3. 获取指定交易详情 `get_transaction_receipt_for_hash(hash)`

- 样例
```rust

    // New BlockScanner
    let bs = BlockScanner {
        url: vec!["https://cloudflare-eth.com/".to_string(), "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string()],
        retry: Default::default(),
        id: "1".to_string(),
    };

    // Get latest number
    let latest_number = bs.get_block_latest_number().await.unwrap();

    // Get block info by latest number
    let block_info = bs.get_transactions_for_block(latest_number).await.unwrap();

    // Get transaction info by 0x80fdaa7f5f54cbe28b84f41afb9543cf0c9eb0d9f4b8a620c2fb5faf0b1c2810
    let transaction_info = bs.get_transaction_receipt_for_hash("0x80fdaa7f5f54cbe28b84f41afb9543cf0c9eb0d9f4b8a620c2fb5faf0b1c2810").await.unwrap();

```


