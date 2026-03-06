# Design Doc

## Backend

### Gestión de comunicación MQTT
El objetivo del gestor es tener un control de todo el entorno de MQTT. Esto conlleva todo el tema de seguridad en la comunicación, gestionar las publicaciones y las suscripciones y controlar las conexiones que se realizanEl objetivo del gestor es tener un control de todo el entorno de MQTT. Esto conlleva todo el tema de seguridad en la comunicación, gestionar las publicaciones y las suscripciones y controlar las conexiones que se realizan.
Para ello, se van a necesitar los siguientes servicios:

- Servicio de publicación de mensajes.
- Servicio de recepción de mensajes utilizando Channels o algún otro método eficiente para subir los mensajes
- Servicio de manejo de mensajes con el que tomar decisiones.
- Servicio de conexión o de nueva conexión. Puede ser necesario una repositorio de clientes, pendiente de analizar
- Servicio de desconexión.

### Gestor de comunicación Bluetooth
Nos podemos encontrar con dispositivos cuya comunicación se realiza por bluetooth en lugar de MQTT u otro protocolo. Por ello es necesario tener soporte para el descubrimiento de nuevos dispositivos, la conexión con los mismos y el envío y recepción de mensajes. Hay que tener soporte tanto para BLE como Bluetooth normal o cualquiera de sus versiones.
Los servicios son similares a los de MQTT pero hay que añadir el **servicio de descubrimiento**.

### Implementación de comunicación con Prometheus
Prometheus se usará como base de datos de series temporales para que el usuario pueda visualizar los datos en gráficas dentro de la aplicación. Por ello, cuando se reciban nuevos datos desde los sensores, se deben guardar en esta base de datos.


### Implementación de comunicación con MongoDB
Mongo se va a utilizar para guardar la información de los dispositivos, clientes, etc. Es decir, todas las entidades que no tienen nada que ver con los datos temporales. Hay que definir las diferentes entidades que componen el sistema como son protocolos, dispositivos, clientes... y sus propiedades.
