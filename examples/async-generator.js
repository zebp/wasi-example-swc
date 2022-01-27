async function* greeter(max) {
  for (let i = 0; i < max; i++) {
    yield `Hello, world! ${i}`;
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
}

async function main() {
  for await (const msg of greeter(10)) {
    console.log(msg);
  }
}

main();
