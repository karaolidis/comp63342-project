public class Boolean0 {
    public static void test(boolean b) {
        b = !b;
        assert b == false;
    }
}
