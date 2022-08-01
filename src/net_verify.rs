fn check_shipments() {
    todo!();

    // Search for Shipments
    // Get the Shipment
    // Check the Shipment's transaction
    // Sign the Shipment
    // Return the Shipment
}
/*fn check_get_wallet() {
    todo!();
}*/

pub async fn check_all() {
    loop {
        println!("[NETWORK] Checking for new transactions...");
        check_shipments();

        //check_get_wallet();
    }
}