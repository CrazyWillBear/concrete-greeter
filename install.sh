cargo build --release
sudo mv target/release/concrete-greeter /usr/bin/concrete-greeter
echo "Add `concrete-greeter` to the top of your bashrc, zshrc, or other shell rc file"