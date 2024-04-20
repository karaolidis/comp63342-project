public class Float0 {
    public static void test(double i) {
        double j = 10.0 / i;
        assert Double.isFinite(j);
    }
}
