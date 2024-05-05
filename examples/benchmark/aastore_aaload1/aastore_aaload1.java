public class aastore_aaload1 {
  static class A {
    int value = 0;
  }

  public static void test(int size) {
    if (size < 0) return;
    A[] array = new A[size];

    for (int i = 0; i < size; i++) {
      array[i] = new A();
    }
    for (int i = 0; i < size; i++) {
      assert array[i] != null;
    }
  }
}
