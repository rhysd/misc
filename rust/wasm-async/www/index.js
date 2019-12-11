import * as rust from 'wasm-async';

async function main() {
    const ret = await rust.async_fetch_index_html();
    console.log('ret!', ret);

    await rust.async_fetch_index_html_2();

    const opts = rust.FetchOptions.new().userAgent('hello');
    await rust.try_my_fetch(opts);

    window.rust = rust;
}

main().catch(console.error);
