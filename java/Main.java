public class Main {
    public static void main(String[] args) {
        int n = 1_000_000;

        long start = System.currentTimeMillis();

        ArrayList<Sensor> sensors = new ArrayList<>(n);

        for(int i=0;i<n;i++){
            sensors.add(new Sensor(i, ThreadLocalRandom.current().nextFloat()));
        }

        float sum = 0;
        for(Sensor s : sensors) sum += s.value;

        long end = System.currentTimeMillis();

        System.out.println("Runtime: " + (end-start) + " ms");


        HashMap<Integer, Float> map = new HashMap<>(n);

        for(int i=0;i<n;i++)
            map.put(i, (float)i);

        long startLookup = System.nanoTime();
        map.get(500000);
        System.out.println("Lookup: " + (System.nanoTime()-startLookup));
    }
}