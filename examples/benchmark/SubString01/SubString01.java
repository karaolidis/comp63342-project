public class SubString01 {
    public static void test(String letters, String sub1, String sub2, int from1, int to1, int from2) {
        String tmp = letters.substring(from2);
        assert tmp.equals(sub1);
        tmp = letters.substring(from1, to1);
        assert tmp.equals(sub2);
    }
}
