
unsigned int add(unsigned int x, unsigned int y) {
   return x + y;
}

int main() {
   unsigned int x{5};
   int y{10};
   x = y;

   auto z = add(2, -3);
}


