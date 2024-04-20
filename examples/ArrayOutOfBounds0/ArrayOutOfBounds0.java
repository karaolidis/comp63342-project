public class ArrayOutOfBounds0 {
    public static void test(int size) {
        if (size < 0)
            return;

        int[] a = new int[4];
        a[size] = 0;
    }
}
