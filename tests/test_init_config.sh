if [ -f ".autocorrectrc.test" ]; then
  rm .autocorrectrc.test
fi
cargo run -q -- -c .autocorrectrc.test init 
if [ -f ".autocorrectrc.test" ]; then
  rm .autocorrectrc.test
  echo "Ok"
else
  echo "Error: .autocorrectrc.test not created"
  exit 1
fi