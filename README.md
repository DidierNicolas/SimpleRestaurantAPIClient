# SimpleRestaurantApiClient

<p>This project contains the client application.</p>

<h2>Build & Launch the Application</h2>
<p>This application need to be run after the server one. (https://github.com/DidierNicolas/SimpleRestaurantApiServer)<br>
To build and run this application, apply theses instructions</p>
<code>cargo build --bin SimpleRestaurantApiClient </code><br>
<code>cargo run --bin SimpleRestaurantApiClient [num]</code>

<p>[num] is the number of client making requests</p>

<h2>Outputs</h2>
The application will continusly make http request until you stop it.
It will randomly make an http request between :
<h4>CREATE ITEM</h4>
<img width="943" alt="image" src="https://user-images.githubusercontent.com/19792149/168812477-d281db1b-abe5-4aa5-9577-43f83b419693.png">
<p>It will output the content of the created item(s) for the table number</p>

<h4>GET ITEM</h4>


<p>It will output the content of an item for an item number</p>

<h4>UPDATE ITEM</h4>
<img width="555" alt="image" src="https://user-images.githubusercontent.com/19792149/168948625-9ecc821d-0c84-4986-bbc4-f9c85a5cf23c.png">
<p>It will output the content of the updated item for an item number</p>

<h4>DELETE ITEM</h4>
<img width="547" alt="image" src="https://user-images.githubusercontent.com/19792149/168948545-c53fadc3-cecd-426a-828c-cc58c6a3991a.png">
<p>It will output 1 if an item has been deleted. <br> It will output 0 if no item has been deleted</p>

<h4>LIST ITEMS FOR A TABLE</h4>


<p>It will output the content of all items for a table number</p>


