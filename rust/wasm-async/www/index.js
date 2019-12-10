import * as rust from 'wasm-async';

async function main() {
    const ret = await rust.async_fetch_index_html();
    console.log('ret!', ret);

    await rust.async_fetch_index_html_2();
}

main().catch(console.error);
