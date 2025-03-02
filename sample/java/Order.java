package java;

/**
 * @ubiquitous Order
 * @context E-commerce
 * @description Represents a customer's purchase order.
 */
public class Order {
    private String name;

    /**
     * Constructor
     * @param name The name
     */
    public Order(String name) {
        this.name = name;
    }

    /**
     * Get the name
     * @return The name
     */
    public String getName() {
        return name;
    }
}
