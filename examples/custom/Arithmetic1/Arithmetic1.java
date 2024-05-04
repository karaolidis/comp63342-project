public class Arithmetic1 {
  public static void test(int i, int j) {
    int a = 10 / i;
    int b = j + 1;

    @SuppressWarnings("unused")
    int c = b / a;

    @SuppressWarnings("unused")
    int d = a / b;
  }
}
