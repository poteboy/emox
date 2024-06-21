// @ts-check
import { getCssInfo } from "@css-lens/wasm";

const main = async () => {
  const cssInfo = getCssInfo(`
        .container {
            display: flex;
            justify-content: center;
            align-items: center;
            font-size: 16px;
            width: 100%;
            margin: 0 auto;
        }
    `);

  console.log(JSON.stringify(cssInfo, null, 2));
};

main();
