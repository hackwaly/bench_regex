///|
fn scan_words(
  input : StringView,
  words : Int,
) -> Int {
  lexmatch input {
    (_, "\w+" as _word, next) => scan_words(next, words + 1)
    _ => words
  }
}

///|
fn scan_words_bytes(
  input : BytesView,
  words : Int,
) -> Int {
  lexmatch input {
    (_, "\w+" as _word, next) => scan_words_bytes(next, words + 1)
    _ => words
  }
}

///|
const TEST_BYTES = b"Non enim laborum velit commodo deserunt incididunt elit sunt nulla ullamco. Quis ipsum aliqua id mollit minim velit et cupidatat eiusmod nostrud pariatur duis irure ad. Dolore enim et elit fugiat.\nNostrud elit minim aute qui labore id aute aute ea nostrud cupidatat. Aliquip et commodo anim dolor nostrud voluptate proident. Voluptate tempor amet consequat nisi excepteur aute anim aute. Sunt ipsum tempor esse consequat cupidatat.\nIpsum minim cillum adipisicing incididunt incididunt qui non excepteur mollit qui. Non aute sunt dolore eu sunt ea aute nisi dolor eiusmod. Fugiat id culpa exercitation consectetur cupidatat. Sunt sint nostrud dolor aute sit cupidatat voluptate reprehenderit ut cillum nulla. Dolore sunt elit elit et quis qui.\n"
const TEST_STRING = "Non enim laborum velit commodo deserunt incididunt elit sunt nulla ullamco. Quis ipsum aliqua id mollit minim velit et cupidatat eiusmod nostrud pariatur duis irure ad. Dolore enim et elit fugiat.\nNostrud elit minim aute qui labore id aute aute ea nostrud cupidatat. Aliquip et commodo anim dolor nostrud voluptate proident. Voluptate tempor amet consequat nisi excepteur aute anim aute. Sunt ipsum tempor esse consequat cupidatat.\nIpsum minim cillum adipisicing incididunt incididunt qui non excepteur mollit qui. Non aute sunt dolore eu sunt ea aute nisi dolor eiusmod. Fugiat id culpa exercitation consectetur cupidatat. Sunt sint nostrud dolor aute sit cupidatat voluptate reprehenderit ut cillum nulla. Dolore sunt elit elit et quis qui.\n"

// ///|
// extern "js" fn wordcount_js(input : String) -> Unit =
//   #|(input) => {
//   #|  let words = 0;
//   #|  let cursor = 0;
//   #|  const regex = /(\w+)/g;
//   #|  while (true) {
//   #|    if (cursor >= input.length) {
//   #|      break;
//   #|    }
//   #|    regex.lastIndex = cursor;
//   #|    const match = regex.exec(input);
//   #|    if (match === null) {
//   #|      break;
//   #|    }
//   #|    words += 1;
//   #|    cursor = regex.lastIndex;
//   #|  }
//   #|}

///|
test (b : @bench.T) {
  b.bench(name="lexmatch", () => {
    let _ = scan_words(TEST_STRING, 0)

  })
  b.bench(name="lexmatch_bytes", () => {
    let _ = scan_words_bytes(TEST_BYTES, 0)

  })
  // b.bench(name="js_regex", () => {
  //   let _ = wordcount_js(TEST_STRING)
  // })
}

///|
test {
  let words = scan_words(TEST_STRING, 0)
  println("\{words}")
  // let _ = wordcount_js(TEST_STRING)
}
