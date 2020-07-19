#!/bin/sh

# TODO: this script is NOT tested yet!
run() {
    echo "================================="
    echo "1. update pq"
    echo "2. delete old database"
    echo "================================="
    read -p "Input your choice: " choice
    case ${choice} in
    1) update_pq ;;
    2) del_old_data ;;
    *)
        echo "invalid option"
        ;;
    esac
}

update_pq() {
  read -p 'Old version number:' OLD_VER
  read -p 'New version number:' NEW_VER

  # stop service, to avoid update errors
  echo "shuting down services..."
  docker-compose down # stop

  echo "updating database..."
  cd pg
  # create a temporary directory for updating
  mkdir -p tmp/$OLD_VER
  mkdir -p tmp/$NEW_VER
  sudo cp -rf data tmp/$OLD_VER
  sudo mv -r data data_old

  # update database
  docker pull tianon/postgres-upgrade
  docker run --rm
    -v tmp:/var/lib/postgresql \
    tianon/postgres-upgrade:$OLD_VER-to-$NEW_VER \
    --link

  # copy new data back
  echo "copying new database files..."
  cp -rf tmp/$NEW_VER/data ./
  cp -f tmp/analyze_new_cluster.sh update-utils/
  cp -f tmp/delete_old_cluster.sh update-utils/

  echo "starting up services..."
  cd ..
  docker-compose up -d

  echo "connecting to pg service..."
  echo "to analyze cluster or delete old cluster, run scripts in /mnt"
  docker exec -it pg /bin/sh

  cd ..
}

del_old_data() {
  echo "deleting old database files..."
  rm -rf data_old
}