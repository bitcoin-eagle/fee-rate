use bitcoincore_rpc::{
    bitcoin::{Amount, BlockHash, Denomination},
    bitcoincore_rpc_json::GetBlockHeaderResult,
    json::{GetBlockTemplateCapabilities, GetBlockTemplateModes, GetBlockTemplateRules},
    Client, RpcApi,
};

fn main() -> anyhow::Result<()> {
    run()
}

fn run() -> anyhow::Result<()> {
    let c = Client::new(
        "localhost:8332",
        bitcoincore_rpc::Auth::UserPass("rust-utxos".into(), "o4ka4wx3i0wxar0bec2w1sm9h".into()),
    )?;
    // let h = c.get_best_block_hash()?;
    // println!("best block hash: {:?}\n", h);
    // let m = c.get_mempool_info()?;
    // println!("mempool info: {:?}\n", m);
    let m = c.get_raw_mempool()?;
    let m_v = c.get_raw_mempool_verbose()?;
    for txid in m {
        let tx = &m_v[&txid];
        let fee_rate_base = tx.fees.base.to_sat() as f64 / tx.vsize as f64;
        let fee_rate_ancestor = tx.fees.ancestor.to_sat() as f64 / tx.ancestor_size as f64;
        if 1 < tx.ancestor_count {
            println!(
                "{} base: {:.3} sat/vB ancestor: {:.3} sat/vB ancestors: {}",
                txid, fee_rate_base, fee_rate_ancestor, tx.ancestor_count
            );
            println!("{:?}", tx);
        }
    }
    let template = c.get_block_template(
        GetBlockTemplateModes::Template,
        &[
            GetBlockTemplateRules::SegWit,
            GetBlockTemplateRules::Csv,
            GetBlockTemplateRules::Taproot,
        ],
        &[],
    )?;
    let weight_limit = template.weight_limit as usize;
    let weight_half = weight_limit / 2;
    let mut high_priority = None::<f64>;
    let mut weight_acc = 0_usize;
    for tx in template.transactions {
        weight_acc += tx.weight;
        if weight_half <= weight_acc {
            high_priority = Some((tx.fee.to_sat() * 4) as f64 / tx.weight as f64);
            break;
        }
    }
    println!("high priority: {:?}\n", high_priority);
    // c.send_raw_transaction(tx)
    // println!("template: {:?}\n", t);
    // let t_0 = t.transactions.first();
    // println!("first tx: {:?}\n", t_0);
    // let t_l = t.transactions.last();
    // println!("last tx: {:?}\n", t_l);
    Ok(())
}
