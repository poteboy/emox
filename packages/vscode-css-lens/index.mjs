import { getCssInfo } from "@css-lens/wasm";

const main = async () => {
  const cssInfo = getCssInfo(`
        .container, div > span {
            display: flex;
            justify-content: center;
            align-items: center;
            font-size: 16px;
            width: 100%;
        }
    `);

  console.log(cssInfo.rules[0].declarations);
};

main();
