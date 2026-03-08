# MQTT Design Doc

## Connection
Este servicio se encarga de de manejar la conexion con el broker MQTT. Su funcion es establecer una conexion con el broker de mensajes y, si es necesario, devolver el objeto que maneja esa conexion para que el resto de servicios puedan publicar mensajes o suscribirse a nuevos topics. El proceso de conexion debe ser sincrono sin bloquear el hilo.

En cuanto al desarrollo, el objetivo es realizarlo de tal forma que el manejo de la libreria se quede a nivel de infraestructura, de tal forma que, si el dia de manana aparece una libreria que lo gestione mejor, pues sustituir  sin tener que cambiar el codigo, solamente la parte especifica de esa nueva libreria.

Este servicio debe recibir un manejador de conexion, y una IP y un puerto y, de esta forma, conectarse con el server. La implementacion de la conexion debe tener en cuenta que si la conexion ya esta realizada, no es necesario conectar. Tambien hay que manejar un evento de desconexion, donde, por alguno de los casos, se pierda la conexion con el broker.

## Disconnection

## Suscribers

## Publishers
