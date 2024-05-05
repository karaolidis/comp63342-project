class recursion2 {
  public static void test(int depth) {
    assert recursion_test(depth, 0) == 10;
  }

  public static int recursion_test(int depth, int currentDepth) {
    if (currentDepth < depth) return recursion_test(depth, currentDepth + 1) + 1;
    else return 0;
  }
}
