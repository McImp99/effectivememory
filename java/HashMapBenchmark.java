package org.example;

import java.util.*;
import java.util.concurrent.ThreadLocalRandom;

class Sensor {
    int id;
    float value;

    Sensor(int id, float value) {
        this.id = id;
        this.value = value;
    }
}

public class Main {
    public static void main(String[] args) {
        int n = 1_000_000;

        
        long startBuild = System.currentTimeMillis();
        HashMap<Integer, Float> map = new HashMap<>(n);
        for(int i = 0; i < n; i++) {
            map.put(i, (float)i);
        }
        long buildTime = System.currentTimeMillis() - startBuild;

        
        long startLookup = System.nanoTime();
        map.get(500_000);
        long lookupTime = System.nanoTime() - startLookup;

        System.out.println("Java HashMap:");
        System.out.println("Build time: " + buildTime + " ms");
        System.out.println("Lookup time: " + lookupTime + " ns");
    }
}
