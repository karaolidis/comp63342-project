public class Float0 {
    public static void test(float i) {
        float j = 10.0f / i;
        assert Float.isFinite(j);
    }
}
